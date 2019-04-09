import Foundation;

public enum ColorStyle {
    {% for rect in rects %}
        static let {{rect.name|slugify|replace(from="-", to="_")|camelcase}} = UIColor(red: {{rect.fills[0].color.r}}, green: {{rect.fills[0].color.g}}, blue: {{rect.fills[0].color.b}}, alpha: {{rect.fills[0].color.a}})
    {% endfor %}
}
