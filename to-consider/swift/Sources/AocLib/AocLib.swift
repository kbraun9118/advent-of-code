import Foundation

public func readInputFile(_ day: String) -> [String] {
  let file = try! String(contentsOfFile: "./input/\(day)/input.txt")
  return file.split(whereSeparator: \.isNewline).compactMap(String.init)
  }

public func readTestFile(_ day: String) -> [String] {
  let file = try! String(contentsOfFile: "./input/\(day)/test.txt")
  return file.split(whereSeparator: \.isNewline).compactMap(String.init)

}
