day1 <- function(list1, list2) {
    list1 <- sort(list1)
    list2 <- sort(list2)

    difference <- sum(abs(list1-list2))

    score <- 0
    i <- 1
    j <- 1

    while((i <= length(list1)) && (j <= length(list2))) {
        if (list1[i] < list2[j]) {
            i <- i + 1
        } else if (list1[i] == list2[j]) {
            value <- list1[i];
            i <- i + 1
            j <- j + 1
            list1_counter <- 1
            list2_counter <- 1
            
            if (i <= length(list1) && j <= length(list2)) {
                while(i <= length(list1) && list1[i] == value) {
                    list1_counter <- list1_counter + 1
                    i <- i + 1
                }
                while(j <= length(list2) && list2[j] == value) {
                    list2_counter <- list2_counter + 1
                    j <- j + 1
                }
            }
            
            score <- score + list1_counter * list2_counter * value;
        } else {
            j <- j + 1
        }
    }
    return(list(difference=difference, score=score))
}

inputs <- sapply(strsplit(readLines(con = "stdin"), "\\s+"), as.numeric)
print(day1(inputs[1,], inputs[2,]))