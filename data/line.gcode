%
G21         ; Set units to mm
G90         ; Absolute positioning
G64 P0.1 Q0.02
M4 S1000
M68 E0 Q80

;
; Operation:    0
; Type:         Mill Cut
; Paths:        1
; Direction:    Conventional
; Rapid Z:      0
; Start Z:      0
; End Z:        -1
; Pass Depth:   1
; Plunge rate:  1000 mm/min
; Cut rate:     500 mm/min
;
; Retract
G0 Z0.000

; Path 0
; Rapid to initial position
G0 X10.000 Y10.000
G0 Z0.000
; plunge
G1 Z-1.000 F1000 S10000
; cut
G1 X80.000 Y10.000 F500 S10000
; Retract
G0 Z0.000
M5          ; Switch tool offEnd
%
