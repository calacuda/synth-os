default:
  @just -l

vital-toggle-second-midi-input ID:
  xdotool mousemove -w {{ID}} 91 35 click 1 sleep 0.01 mousemove -w {{ID}} 840 848 click 1 sleep 0.01 mousemove -w {{ID}} 91 35 click 1
