package day06

import day06.Orbitals.OrbitalTree
import org.scalatest.freespec.AnyFreeSpec
import upickle.default._

import scala.io.Source

class OrbitalTest extends AnyFreeSpec {
  val inputFileName = "input_example1"
  val (inputFileNameTxt, inputFileNameParsed) = (s"$inputFileName.txt", s"${inputFileName}_parsed.txt")

  val inputFileTxtContents = Source.fromResource(inputFileNameTxt).mkString("")
  val inputFileParsedContents = Source.fromResource(inputFileNameParsed).mkString("")

  s"when fileLines is set to the contents of $inputFileNameTxt" - {
    """Orbitals.toOrbitalTree(fileLines, "COM")""" - {
      s"should build the OrbitalTree in test/resources/$inputFileNameParsed?" in {
        val actualOrbitalTree = Orbitals.toOrbitalTree(inputFileTxtContents.split("\n"), "COM").get
        val expectedOrbitalTree = read[OrbitalTree](inputFileParsedContents)

        assert(actualOrbitalTree == expectedOrbitalTree)
      }
    }

    "orbitalTree.sumOfDepths()" - {
      val expectedSumOfDepths = 42
      s"should return $expectedSumOfDepths" in {
        val orbitalTree = Orbitals.toOrbitalTree(inputFileTxtContents.split("\n"), "COM").get

        assert(orbitalTree.sumOfDepths() == expectedSumOfDepths)
      }
    }
  }

  val inputFileName2Txt = s"input_example2.txt"
  s"when orbitalTree is parsed from the contents of $inputFileName2Txt" - {
    val inputFile2TxtContents = Source.fromResource(inputFileName2Txt).mkString("")
    val orbitalTree = Orbitals.toOrbitalTree(inputFile2TxtContents.split("\n"), "COM").get

    """orbitalTree.pathTo("YOU")""" - {
      val expectedPathTo = List("COM", "B", "C", "D", "E", "J", "K", "YOU")
      s"should be $expectedPathTo" in {
        assert(orbitalTree.pathTo("YOU").map(_.name) == expectedPathTo)
      }
    }

    "orbitalTree.pathLengthBetween(name1, name2)" - {
      val expectedPathLengthBetween = 4;
      s"should return $expectedPathLengthBetween" in {

        assert(orbitalTree.pathLengthBetween("YOU", "SAN") == expectedPathLengthBetween)
      }
    }
  }
}