(defun foo (value)
    ; (declare (optimize (speed 3) (debug 0)))
    (+ value value))



(format t "~S~%" (labels (
    (subtract (a b)
        (- a b))
    (add (a b) 
        (+ a b)))
            (add 4 (subtract 3 1))))


; 
; 
; 
; 
	(defun my-map (list fn)
	    (labels ((my-map-inner (remaining)
	            ; when lets us handle nil recursive case easier
	            (when remaining
	                ; funcall -> look up in variable namespace
	                (let ((new-value (funcall fn (car remaining))))
	                    (cons 
	                        new-value 
	                        (my-map-inner (cdr remaining)))))))
	        (my-map-inner list)))
	 
	(defun add-one (value)
	    (+ value 1))
    
(defun add-one (value)
    (+ value 1))

(defparameter *original-list* '(1 2 3 4 5))




(let ((new-list (mapcar 
    #'(lambda (value) (+ value 1)) 
    *original-list*)))
        (format t "New List was: ~S~%" new-list))
    

; (format t "New List: ~S~%" (my-map '(1 2 3 4 5) #'add-one))