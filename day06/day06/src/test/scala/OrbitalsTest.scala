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

  """Orbitals.toOrbitalTree(fileLines, "COM")""" - {
    s"when fileLines is set to the contents of $inputFileName" - {
      s"should build the OrbitalTree in test/resources/$inputFileNameParsed?" in {
        val actualOrbitalTree = Orbitals.toOrbitalTree(inputFileTxtContents.split("\n"), "COM").get
        val expectedOrbitalTree = read[OrbitalTree](inputFileParsedContents)

        assert(actualOrbitalTree == expectedOrbitalTree)
      }
    }
  }

  "Orbitals.sumOfDepths(orbitalTree)" - {
    s"when orbitalTree is parsed from the contents of $inputFileName" - {
      val expectedSumOfDepths = 42
      s"should return $expectedSumOfDepths" in {
        val orbitalTree = Orbitals.toOrbitalTree(inputFileTxtContents.split("\n"), "COM").get

        assert(Orbitals.sumOfDepths(orbitalTree) == expectedSumOfDepths)
      }
    }
  }
}