@echo off


curl -X PUT localhost/notes -H "Content-Type: application/json" -d "{  \"title\": \"My Note\", \"text\": \"This is the content of the note 2.\", \"date\": \"foo\"}"

echo.
