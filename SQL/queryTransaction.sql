BEGIN TRANSACTION;

SELECT string, arraySize
FROM InputMessages
WHERE id = <id>;

SELECT arrayIndex, intValue
FROM ArrayElements
WHERE messageId = <id>;

DELETE FROM ArrayElements
WHERE messageId = <id>;

DELETE FROM InputMessages
WHERE id = <id>;

COMMIT;