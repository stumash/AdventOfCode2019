package day06

import scala.collection.mutable.ArrayBuffer
import scala.io.StdIn

object Main {
  def main(args: Array[String]): Unit = {
    println(Orbitals.toOrbitalTree(readStdinLines(), "COM").map(_.pathLengthBetween("YOU", "SAN")))

    def readStdinLines(): Iterable[String] = {
      val lines = ArrayBuffer[String]()

      var currLine: String = StdIn.readLine()
      while (currLine != null) {
        lines.append(currLine)
        currLine = StdIn.readLine()
      }

      lines
    }
  }
}
