; Code injections for Razen

; F-string interpolation
(interpolation
  (identifier) @content
  (#set! injection.language "razen"))

; Comments with code examples
((comment) @content
  (#match? @content "```razen")
  (#set! injection.language "razen"))
