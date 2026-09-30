; (defparameter *name* (progn 
;     (format t "Hello world~%")
;     (format t "What is your name?~%")
;     (finish-output)
;     (read-line)))



; (progn 
;     (format t "What is your name~%")
;     (finish-output)
;     (defparameter *name* (read-line)))

; (format t "Name: ~S~%" *name*)

(defun get-letter-grade (score)
	    (cond 
	        ((>= score 90) 
                (format t "Excellent~%")   "A")
	        ((>= score 80) (format t "Good Job~%")    "B")
	        ((>= score 70) (format t "...~%")         "C")
	        (t             (format t ":(~%")          "F")))

(format t "~S~%" (get-letter-grade 39))


(defun my-sqrt (value)
    (unless (< value 0)
        (format t "square rooting the number")
        (sqrt value)))

; (format t "~S~%" (my-sqrt 64))

; (defun foo (value)
;     (when (> value 0)
;         (format t "You gave a positive number~%")
;         (* value 2)))


; (defun square (value)
;     (format t "calling square function~%")
;     (* value value))

; (defun main ()
;     (format t "Starting main...~%")
;     (format t "What is your name~%"))

; (format t "~S~%" 
;     (if (> 7 5)
;         (progn 
;             (format t "If statement ran")
;             `(BOB))))

; (main)