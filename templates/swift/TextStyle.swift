import Foundation;

public enum TextStyle {
    {% for text in texts %}
        static let {{text.name|slugify|replace(from="-", to="_")|upper}} = UIFont.systemFont(ofSize: {{text.style.fontSize}}, weight: .regular) ?? UIFont.systemFont(ofSize: {{text.style.fontSize}}, weight: .regular)
    {% endfor %}
}
