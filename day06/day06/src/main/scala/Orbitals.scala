package day06

import scala.collection.mutable.{ArrayDeque, HashMap}
import scala.util.{Failure, Success, Try}
import upickle.default.{ReadWriter, macroRW}

object Orbitals {
  sealed trait OrbitalTree {
    def sumOfDepths(depth: Int = 0): Int
    def pathTo(name: String): List[OrbitalNode]
    def pathLengthBetween(name1: String, name2: String): Int = {
      def _pathLengthBetween(nodes1: List[OrbitalNode], nodes2: List[OrbitalNode]): Int = {
        (nodes1, nodes2) match {
          case (h1::tail1, h2::tail2) if h1 == h2 => _pathLengthBetween(tail1, tail2)
          case (_, _) => nodes1.length + nodes2.length
        }
      }

      _pathLengthBetween(pathTo(name1).init, pathTo(name2).init)
    }
  }
  object OrbitalTree {
    implicit val rw_OrbitalTree: ReadWriter[OrbitalTree] = macroRW
  }

  case class OrbitalNode(name: String, orbitees: List[OrbitalTree]) extends OrbitalTree {
    override def sumOfDepths(depth: Int = 0): Int = {
      depth + orbitees.map(_.sumOfDepths(depth + 1)).foldLeft(0)(_ + _)
    }
    override def pathTo(name: String): List[OrbitalNode] = {
      if (name == this.name) {
        this :: Nil
      } else {
        orbitees.map(_.pathTo(name)).find(_ != Nil) match {
          case None => Nil
          case Some(path) => this :: path
        }
      }
    }
  }
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
}
