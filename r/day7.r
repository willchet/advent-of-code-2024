exact_quot <- function(x, y) {
    if (y == 0) {
        return(NULL)
    } else if (x %% y == 0) {
        return(x / y)
    } else {
        return(NULL)
    }
}

checked_trunc <- function(x, y) {
    y_string <- as.character(y)
    if (endsWith(as.character(x), y_string)) {
        return((x - y) / 10 ^ nchar(y_string))
    } else {
        return(NULL)
    }
}

is_solvable <- function(eq, current_index, total) {
    if (current_index == 1) {
        if (eq[[1]] == total) {
            return("with_arith")
        } else {
            return("no")
        }
    } 
    quot <- exact_quot(total, eq[current_index])
    if (!is.null(quot)) {
        eq_solvable <- is_solvable(eq, current_index - 1, quot)
        if (eq_solvable != "no") {
            return(eq_solvable)
        }
    }
    diff <- total - eq[current_index]
    if (diff >= 0) {
        diff_eq_solvable <- is_solvable(eq, current_index - 1, diff);
        if (diff_eq_solvable != "no") {
            return(diff_eq_solvable)
        }
    }
    trunc = checked_trunc(total, eq[current_index])
    if (!is.null(trunc) && is_solvable(eq, current_index - 1, trunc) != "no") {
        return("with_concat")
    }
    return("no")
}

day7 <- function(eqs) {
    with_arith_count <- 0
    with_concat_count <- 0
    for (eq in eqs) {
        solvability <- is_solvable(eq$args, length(eq$args), eq$total)
        if (solvability == "with_arith") {
            with_arith_count <- with_arith_count + eq$total
            with_concat_count <- with_concat_count + eq$total
        } else if (solvability == "with_concat") {
            with_concat_count <- with_concat_count + eq$total
        }
    }
    c(with_arith_count, with_concat_count)
}

options(scipen=999)
input <- lapply(readLines(con = "stdin"), function(line) {
    parts <- strsplit(line, ": ")[[1]]
    total <- as.numeric(parts[[1]])
    args <- as.numeric(unlist(strsplit(parts[[2]], "\\s+")))
    list(total = total, args = args)
})
print(day7(input))