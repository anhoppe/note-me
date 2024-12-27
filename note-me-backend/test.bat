@echo off


curl -X PUT https://note-me-backend-991989948061.us-central1.run.app/notes -H "Content-Type: application/json" -d "{  \"title\": \"My Note\", \"text\": \"This is the content of the note 2.\", \"createdAt\": \"foo\"}"

echo.
