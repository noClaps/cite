import ArgumentParser
import Foundation

@main
struct cite: AsyncParsableCommand {
    @Argument(help: "The DOI identifier or URL to fetch data from.")
    var url: String

    mutating func run() async throws {
        let doi = "10.\(url.split(separator: "10.", maxSplits: 1).last!)"
        let reference = try await CrossRef.fetch(doi: doi)
        print(reference)
    }
}

struct Reference: Decodable, CustomStringConvertible {
    struct Published: Decodable, CustomStringConvertible {
        var dateParts: [[Int?]]

        enum CodingKeys: String, CodingKey {
            case dateParts = "date-parts"
        }

        var description: String {
            var date = String(dateParts[0][0]!).padLeft(toLength: 4, withPad: "0")
            if dateParts[0].count > 1, let month = dateParts[0][1] {
                date += "-" + String(month).padLeft(toLength: 2, withPad: "0")
                if dateParts[0].count > 2, let day = dateParts[0][2] {
                    date += "-" + String(day).padLeft(toLength: 2, withPad: "0")
                }
            }
            return date
        }
    }

    struct Author: Decodable, CustomStringConvertible {
        var given: String?
        var family: String?

        var description: String {
            if given == nil || family == nil {
                return ""
            }
            return "- \(family!), \(given!)"
        }
    }

    enum CodingKeys: String, CodingKey {
        case containerTitle = "container-title"
        case issue = "issue"
        case doi = "DOI"
        case published = "published"
        case page = "page"
        case title = "title"
        case volume = "volume"
        case author = "author"
    }

    var containerTitle: [String]
    var issue: String?
    var doi: String
    var published: Published
    var page: String?
    var title: [String]
    var volume: String
    var author: [Author]

    var description: String {
        """
          type: "article"
          title: "\(title[0])"
          author:
          \(author.filter{!$0.description.isEmpty}.map{"  \($0)"}.joined(separator: "\n  "))
          date: "\(published)"\(page != nil ? "\n  page-range: \(page!)" : "")
          serial-number:
            doi: "\(doi)"
          parent:
            volume: \(volume)\(issue != nil ? "\n    issue: \(issue!)" : "")
            title: "\(containerTitle[0])"
        """
    }
}

extension String {
    func padLeft(toLength: Int, withPad char: Character) -> String {
        let padLength = toLength - self.count
        return repeatElement(char, count: padLength) + self
    }
}

struct CrossRef: Decodable {
    var message: Reference

    static func fetch(doi: String) async throws -> Reference {
        let url = URL(string: "https://api.crossref.org/works/\(doi)")!
        let (data, _) = try await URLSession.shared.data(from: url)
        return try JSONDecoder().decode(Self.self, from: data).message
    }
}
