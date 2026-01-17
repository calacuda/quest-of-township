_:
  @just -l

_import_a_map cp_from cp_to:
  mv ~/Downloads/{{cp_from}} ./assets/maps/{{cp_to}}

import-maps:
  # mv ~/Downloads/qot-starter-town.json ./assets/maps/starter-town.json
  @just _import_a_map qot-starter-town.json starter-town.json
