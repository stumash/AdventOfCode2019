package main

import scala.io.StdIn.readLine
import scala.util.{Failure, Success, Try}
import scala.util.control.Breaks._

object Main {
  def main(args: Array[String]): Unit = {
    val result = for {
      data <- readData()
      result <- runProgram(data)
    } yield result

    println(result)
  }

  def readData(): Try[Array[Int]] = {
    try {
      Success(
        readLine().split(",").map(_.toInt)
      )
    }
    catch { case _: Exception => Failure(new Exception("could not parse first line of stdin")) }
  }

  def readDatum(prompt: String = ""): Int = {
    print(prompt)
    readLine().toInt
  }

  def runProgram(data: Array[Int]): Try[Int] = {
    try {
      breakable {
        var i = 0
        while (i < data.length) {
          val opCode = data(i)
          val (p2Mode, p1Mode) = (opCode %@ 3, opCode %@ 2)
          opCode % 100 match {
            case 1 => {
              data(data(i + 3)) = p1Mode.ifPosition(data(i+1), data(_)) + p2Mode.ifPosition(data(i + 2), data(_))
              i += 4
            }
            case 2 => {
              data(data(i + 3)) = p1Mode.ifPosition(data(i+1), data(_)) * p2Mode.ifPosition(data(i + 2), data(_))
              i+=4
            }
            case 3 => {
              data(data(i+1)) = (readDatum())
              i += 2
            }
            case 4 => {
              println(p1Mode.ifPosition(data(i+1), data(_)))
              i += 2
            }
            case 99 => break
            case _ => throw new Exception(s"invalid op code ${opCode.toString}")
          }
        }
      }
      Success( data(0) )
    }
    catch { case e: Throwable => Failure(e) }
  }

  implicit class IntDigitGetter(i: Int) {
    require(i >= 0)

    def %@(powerOfTen: Int): Int = {
      (i / math.pow(10, powerOfTen).toInt) % 10
    }
  }

  implicit class ParamMode(i: Int) {
    require(i == 0 || i == 1)

    trait Mode
    object Mode {
      object Position extends Mode
      object Immediate extends Mode
    }

    val mode: Mode = i match {
      case 0 => Mode.Position
      case 1 => Mode.Immediate
    }

    def ifPosition(j: Int, f: Int => Int): Int = {
      mode match {
        case Mode.Immediate => j
        case Mode.Position => f(j)
      }
    }
  }
}

