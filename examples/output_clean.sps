* SPSS Syntax for Clinical Interactions & Trainee Well-Being
* Generated: 2026-01-03 13:13:11

GET DATA
  /TYPE=TXT
  /FILE='.\examples\output_clean.csv'
  /DELIMITERS=","
  /FIRSTCASE=2
  /VARIABLES=
    emotional_exhaustion_total F8.2
    emotional_exhaustion_mean F8.2
    alliance_total_total F8.2
    alliance_total_mean F8.2
    supervision_rapport_total F8.2
    supervision_rapport_mean F8.2
    depersonalization_total F8.2
    depersonalization_mean F8.2
    peer_support_total F8.2
    peer_support_mean F8.2
    .
EXECUTE.

* Add variable labels and value labels as needed
