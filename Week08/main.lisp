; '(format t "hello")

; (format t "list was: ~S~%" '(BOB CHARLIE))
; (format t "list was: ~S~%" `(BOB CHARLIE))

(let ((user "Reese"))
    `(HELLO ,user))

(let ((value '(2 3)))
    `(1 ,@value, 4))

(defmacro greet (name)
    `(format t "Hello, ~S~%" ,name))

; (greet "Reese")
; (format t "Hello, ~S~%" "Reese"))

(defmacro with-logging (&body body)
    `(progn
        (format t "Begin code execution~%")
        ,@body
        (format t "End code execution~%")))

(with-logging 
    (let ((name "Reese"))
        (format t "Hi ~S~%" name)))

(defmacro my-when (test &body body)
    `(if ,test
        (progn 
            ,@body)))


(defmacro while (test &body body)
    `(if ,test 
        (progn
            ,@body
            (while ,test ,@body))
        nil))

(format t "~S~%" (macroexpand-1 '(let ((i 0))
    (while (< i 10)
        (format t "~S~%" i)
        (setf i (+ i 1))))))
        
; (let ((input (read)))
;     (my-when (eq input 'TRUE)
;         (format t "You entered true~%")))

; (my-when (TEST) 
; (()
; ()
; ()))

; (format t "Macro expanded to ~S~%"
;     (macroexpand-1 '(with-logging 
;                         (let ((name "Reese"))
;                             (format t "Hi ~S~%" name)))))

; begin code execution
; all of the code
; end code execution