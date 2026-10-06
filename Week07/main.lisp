(defun foo (value)
    ; (declare (optimize (speed 3) (debug 0)))
    (+ value value))



(format t "~S~%" (labels (
    (subtract (a b)
        (- a b))
    (add (a b) 
        (+ a b)))
            (add 4 (subtract 3 1))))