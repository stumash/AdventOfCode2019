package day06

import scala.collection.mutable.{ArrayDeque, HashMap}
import scala.util.{Failure, Success, Try}
import upickle.default.{ReadWriter, macroRW}

object Orbitals {
  sealed trait OrbitalTree
  object OrbitalTree {
    implicit val rw_OrbitalTree: ReadWriter[OrbitalTree] = macroRW
  }
  case class OrbitalNode(name: String, orbitees: List[OrbitalTree]) extends OrbitalTree
  object OrbitalNode {
    implicit val rw_OrbitalNode: ReadWriter[OrbitalNode] = macroRW
  }

  def toOrbitalTree(fileLines: Iterable[String], rootOrbitee: String): Try[OrbitalTree] = {
    def parseMap(fileLines: Iterable[String]): Try[Map[String, List[String]]] = {
      try {
        val map = HashMap[String, ArrayDeque[String]]()
        fileLines.zipWithIndex.foreach({ case (line, i) =>
          line.split("""\)""").toList match {
            case List(orbitee, orbiter) =>
              map(orbitee) = map.getOrElse(orbitee, ArrayDeque()).addOne(orbiter)
            case _ => throw new Exception(s"invalid contents in line $i: $line")
          }
        })
        Success(map.toMap.map({ case (s, ss) => (s, ss.toList) }))
      } catch {
        case e: Throwable => Failure(e)
      }
    }

    def toOrbitalTreeNode(map: Map[String, List[String]], currNodeName: String): OrbitalTree = {
      OrbitalNode(
        currNodeName,
        map.getOrElse(currNodeName, List()).map(toOrbitalTreeNode(map, _))
      )
    }

    for {
      map <- parseMap(fileLines)
    } yield {
      toOrbitalTreeNode(map, rootOrbitee)
    }
  }

  def sumOfDepths(tree: OrbitalTree, depth: Int = 0): Int = {
    tree match {
      case OrbitalNode(_, subTrees) => depth + subTrees.map(sumOfDepths(_, depth+1)).foldLeft(0)(_ + _)
    }
  }
}
