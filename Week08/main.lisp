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


(format t "Macro expanded to ~S~%"
    (macroexpand-1 '(with-logging 
                        (let ((name "Reese"))
                            (format t "Hi ~S~%" name)))))

; begin code execution
; all of the code
; end code execution