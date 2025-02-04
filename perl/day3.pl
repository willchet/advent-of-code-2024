#!/usr/bin/perl
use strict;
use warnings;

my $part1     = 0;
my $part2     = 0;
my $activated = 1;

while ( my $input = <STDIN> ) {
    chomp $input;

    while ( $input =~ /(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))/g ) {
        if ( defined $1 ) {
            $part1 += $2 * $3;
            if ($activated) {
                $part2 += $2 * $3;
            }
        }
        elsif ( defined $4 ) {
            $activated = 1;
        }
        elsif ( defined $5 ) {
            $activated = 0;
        }
    }
}

print "$part1, $part2";
print "\n";
