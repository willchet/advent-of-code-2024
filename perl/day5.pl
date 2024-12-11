#!/usr/bin/perl
use strict;
use warnings;

my $correct_middle = 0;
my $sorted_middle  = 0;

my @rules_matrix;
for my $i ( 0 .. 99 ) {
    for my $j ( 0 .. 99 ) {
        $rules_matrix[$i][$j] = 0;
    }
}

while ( my $input = <STDIN> ) {
    chomp($input);
    if ( $input =~ /(\d{2})\|(\d{2})/ ) {
        $rules_matrix[$1][$2] = 1;
    }
    else {
        last;
    }
}

# For each update
A: while ( my $input = <STDIN> ) {
    chomp($input);
    my @update = split( /,/, $input );

    # For each page in the update
    for my $i ( 0 .. $#update ) {

        # For each subsequent page in the update
        for my $j ( $i + 1 .. $#update ) {

            # If the pair of pages violates a rule
            if ( $rules_matrix[ $update[$j] ][ $update[$i] ] ) {
                my @start_pages = ();
                my %pages_remaining;
                @pages_remaining{@update} = ();

                # Sort the first half (before the middle)
                for my $iter ( 1 .. $#update / 2 ) {
                    my $page = shift(@start_pages);
                    if ( defined $page ) {
                        delete( %pages_remaining{$page} );
                    }
                    else {
                        my @pages = keys(%pages_remaining);

                        # Seek the next page(s) in the sorted order
                      B: for my $page (@pages) {
                            for my $other_page (@pages) {
                                if ( $rules_matrix[$other_page][$page] ) {
                                    next B;
                                }
                            }
                            delete( %pages_remaining{$page} );
                            push( @start_pages, $page );
                        }
                        shift(@start_pages);
                    }

                }

                # Get the middle
                my $page = shift(@start_pages);
                if ( defined $page ) {
                    $sorted_middle += $page;
                }
                else {
                    my @pages = keys(%pages_remaining);

                    # Seek the next page(s) in the sorted order
                  B: for my $page (@pages) {
                        for my $other_page (@pages) {
                            if ( $rules_matrix[$other_page][$page] ) {
                                next B;
                            }
                        }
                        $sorted_middle += $page;
                        last;
                    }
                }

                next A;
            }
        }
    }

    $correct_middle += $update[ $#update / 2 ];
}

print "$correct_middle, $sorted_middle", "\n";
