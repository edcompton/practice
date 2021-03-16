package cinema

fun main() {
    println("Enter the number of rows:")
    val rows = readLine()!!.toInt()

    println("Enter the number of seats in each row:")
    val seatsPerRow = readLine()!!.toInt()

    val seats = rows * seatsPerRow
    val seatMultiDimensionalArray = buildCinema(rows, seatsPerRow)

    executeMenu(seatMultiDimensionalArray, seats, rows)
}

fun executeMenu(seatMultiDimensionalArray: Array<Array<String>>, seats: Int, rows: Int) {
    println("""
1. Show the seats
2. Buy a ticket
3. Statistics
0. Exit""")

    when (readLine()!!) {
        "1" -> showCinema(seatMultiDimensionalArray, seats, rows)
        "2" -> buyTicket(seatMultiDimensionalArray, seats, rows)
        "3" -> runStatistics(seatMultiDimensionalArray, seats, rows)
        "0" -> return
        else -> {
            println("""
Sorry, I didn't recognise that option. 
Please select from the following options;
    
1. Show the seats
2. Buy a ticket
3. Statistics
0. Exit
""")
            executeMenu(seatMultiDimensionalArray, seats, rows)
        }
    }
}

fun runStatistics(seatMultiDimensionalArray: Array<Array<String>>, seats: Int, rows: Int) {
    var numOfPurchasedTickets = 0
    var currentIncome = 0
    for (i in seatMultiDimensionalArray.indices) {
        for (n in seatMultiDimensionalArray[i].indices) {
            if (seatMultiDimensionalArray[i][n] == "B") {
                numOfPurchasedTickets++
                currentIncome += calculateTicketPrice(seats, rows, i)
            }
        }
    }
    val numOfPurchasesPercentage = numOfPurchasedTickets.toDouble() / seats.toDouble() * 100.00
    val totalIncome = showIncome(seats, rows, seats/rows)
    val percentageToPrint = String.format("%.2f", numOfPurchasesPercentage)
    println("""
Number of purchased tickets: $numOfPurchasedTickets
Percentage: $percentageToPrint%
Current income: $$currentIncome
Total income: $$totalIncome
""")

    executeMenu(seatMultiDimensionalArray, seats, rows)
}

fun buyTicket(seatMultiDimensionalArray: Array<Array<String>>, seats: Int, rows: Int) {
    println("\nEnter a row number:")
    val rowNum = readLine()!!.toInt()

    println("Enter a seat number in that row:")
    val seatNum = readLine()!!.toInt()

    try {
        if (seatMultiDimensionalArray[rowNum][seatNum] == "B") {
            throw Exception()
        }
        seatMultiDimensionalArray[rowNum][seatNum] = "B"

        val ticketPrice = calculateTicketPrice(seats, rows, rowNum)
        println("\nTicket price: $$ticketPrice\n")

        executeMenu(seatMultiDimensionalArray, seats, rows)
    } catch (e: IndexOutOfBoundsException) {
        println("Wrong input!")
        buyTicket(seatMultiDimensionalArray, seats, rows)
    }
    catch(e: Exception) {
        println("That ticket has already been purchased!")
        buyTicket(seatMultiDimensionalArray, seats, rows)
    }
}

fun calculateTicketPrice(seats: Int, rows: Int, rowNum: Int): Int {
    return if ((seats > 60 && rowNum <= rows/2) || seats < 60) {
        10
    } else {
        8
    }
}

fun showCinema(seatMultiDimensionalArray: Array<Array<String>>, seats: Int, rows: Int) {
    println("Cinema:")
    for (i in seatMultiDimensionalArray.indices) {
        println(seatMultiDimensionalArray[i].joinToString(" "))
    }

    executeMenu(seatMultiDimensionalArray, seats, rows)
}

fun buildCinema(rows: Int, seatsPerRow: Int): Array<Array<String>> {
    var seatMultiDimensionalArray: Array<Array<String>> = emptyArray()

    for (i in 0..rows) {
        var newSeatArray = emptyArray<String>()
        newSeatArray += if (i == 0) {
            " "
        } else {
            "$i"
        }

        for (n in 1..seatsPerRow) {
            newSeatArray += if (i == 0) {
                n.toString()
            } else {
                "S"
            }
        }
        seatMultiDimensionalArray += newSeatArray
    }

    return seatMultiDimensionalArray
}

fun showIncome(seats: Int, rows: Int, seatsPerRow: Int): Int {
    return if (seats <= 60) {
        seats * 10
    } else {
        val frontHalf = rows / 2
        val frontHalfRevenue = (frontHalf * seatsPerRow) * 10
        val backHalfRevenue = ((rows - frontHalf) * seatsPerRow) * 8
        frontHalfRevenue + backHalfRevenue
    }
}