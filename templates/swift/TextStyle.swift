import Foundation;

public enum TextStyle {
{% for text in texts %}
    {% if text.style.fontPostScriptName == 'HiraginoSans-W6' or text.style.fontPostScriptName == 'HiraKakuPro-W6' or text.style.fontWeight == 700 %}
    static let {{text.name|slugify|replace(from="-", to="_")|camelcase}} = UIFont(name: "{{text.style.fontPostScriptName}}", size: {{text.style.fontSize}}) ?? UIFont.systemFont(ofSize: {{text.style.fontSize}}, weight: .bold)
    {% else %}
    static let {{text.name|slugify|replace(from="-", to="_")|camelcase}} = UIFont(name: "{{text.style.fontPostScriptName}}", size: {{text.style.fontSize}}) ?? UIFont.systemFont(ofSize: {{text.style.fontSize}}, weight: .regular)
    {% endif %}
{% endfor %}
}
