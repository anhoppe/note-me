@echo off


curl -X PUT http://127.0.0.1:3000/notes -H "Content-Type: application/json" -d "{  \"title\": \"My Note\", \"text\": \"This is the content of the note 2.\", \"date\": \"foo\"}"

echo.
