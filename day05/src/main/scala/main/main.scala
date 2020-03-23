package main

import scala.io.StdIn.readLine
import scala.util.{Failure, Success, Try}

import core.IntTools.{IntDigitGetter, ByMode}

object Main {
  def main(args: Array[String]): Unit = {
    val result = for {
      data <- readData()
      result <- runProgram(data)
    } yield result

    result.recover(println(_))
  }

  def readData(): Try[Array[Int]] = {
    try {
      Success(
        readLine().split(",").map(_.toInt)
      )
    }
    catch { case _: Exception => Failure(new Exception("could not parse first line of stdin")) }
  }

  def runProgram(data: Array[Int]): Try[Int] = {
    try {
      var i = 0
      while (i < data.length) {
        val opCode = data(i)
        val (p3Mode, p2Mode, p1Mode) = (opCode %@ 4, opCode %@ 3, opCode %@ 2)
        opCode % 100 match {
          case 1 =>
            data(data(i + 3)) = data(i+1).mapIfPos(p1Mode, data(_)) + data(i+2).mapIfPos(p2Mode, data(_))
            i += 4
          case 2 =>
            data(data(i + 3)) = data(i+1).mapIfPos(p1Mode, data(_)) * data(i+2).mapIfPos(p2Mode, data(_))
            i+=4
          case 3 =>
            data(data(i+1)) = readDatum()
            i += 2
          case 4 =>
            println(data(i+1).mapIfPos(p1Mode, data(_)))
            i += 2
          case 5 =>
            if (data(i+1).mapIfPos(p1Mode, data(_)) != 0) {
              i = data(i+2).mapIfPos(p2Mode, data(_))
            } else {
              i += 3
            }
          case 6 =>
            if (data(i+1).mapIfPos(p1Mode, data(_)) == 0) {
              i = data(i+2).mapIfPos(p2Mode, data(_))
            } else {
              i += 3
            }
          case 7 =>
            data(data(i+3)) =
              if (data(i+1).mapIfPos(p1Mode, data(_)) < data(i+2).mapIfPos(p2Mode, data(_))) {
                1
              } else {
                0
              }
            i += 4
          case 8 =>
            data(data(i+3)) =
              if (data(i+1).mapIfPos(p1Mode, data(_)) == data(i+2).mapIfPos(p2Mode, data(_))) {
                1
              } else {
                0
              }
            i += 4
          case 99 => i = data.length
          case _ => throw new Exception(s"invalid op code ${opCode.toString}")
        }
      }
      Success( data(0) )
    }
    catch { case e: Throwable => Failure(e) }
  }

  def readDatum(prompt: String = ""): Int = {
    print(prompt)
    readLine().toInt
  }
}

