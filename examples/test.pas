FUNCTION Test RETURNS INTEGER
    IF FALSE THEN
        RETURN 2
    ENDIF
    RETURN 2
ENDFUNCTION

PROCEDURE Main
    DECLARE A : INTEGER
    A <- Test()
    OUTPUT A
ENDPROCEDURE