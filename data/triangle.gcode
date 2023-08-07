%
G21         ; Set units to mm
G90         ; Absolute positioning
G64P0.1Q0.02
M4 S1000
M68 E0 Q80

;
; Opeation:    0
; Type:         Mill Cut
; Paths:        3
; Diection:    Conventional
; Rapid Z:      0
; Stat Z:      0
; End Z:        -1
; Pass Depth:   1
; Plunge ate:  1000 mm/min
; Cut ate:     500 mm/min
;
; Retact
G0 Z0.000

; Path 0
; Rapid to initial position
G0 X36.500 Y27.250
G0 Z0.000
; plunge
G1 Z-1.000 F1000 S10000
; cut
G1 X121.000 Y55.750 F500 S10000
; Retact
G0 Z0.000

; Path 1
; Rapid to initial position
G0 X121.000 Y55.750
G0 Z0.000
; plunge
G1 Z-1.000 F1000 S10000
; cut
G1 X142.000 Y16.250 F500 S10000
; Retact
G0 Z0.000

; Path 2
; Rapid to initial position
G0 X142.000 Y16.250
G0 Z0.000
; plunge
G1 Z-1.000 F1000 S10000
; cut
G1 X36.500 Y27.250 F500 S10000
; Retact
G0 Z0.000
M5          ; Switch tool offEnd
%
