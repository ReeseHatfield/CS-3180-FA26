(format t "How old are you?~%")
(finish-output)

(defparameter *age* (read))

(defun square (value)
    "Returns the square of a number"
    (* value value))

(if (>= *age* 35)
    (format t "You are old enough to be president~%")
    (format t "You are NOT old enough to be president~%"))

(format t "Your age squared is ~S~%" (square *age*))
; (defparameter *code* (read))
; (format t "You entered ~S~%" *code*)
; (format t "The type was: ~S~%" (type-of *code*))

; (defparameter *months* (+ 1 2))

; (setf *age* (* 10 3))

; (format t "I am ~S years and ~S months old~%" *age* *months*)