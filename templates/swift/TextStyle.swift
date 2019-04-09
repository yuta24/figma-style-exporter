import Foundation;

public enum TextStyle {
    {% for text in texts %}
        static let {{text.name|slugify|replace(from="-", to="_")|camelcase}} = UIFont(name: "{{text.style.fontPostScriptName}}", size: {{text.style.fontSize}}) ?? UIFont.systemFont(ofSize: {{text.style.fontSize}}, weight: .regular)
    {% endfor %}
}
