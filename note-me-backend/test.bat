@echo off


rem curl -X PUT https://note-me-backend-991989948061.us-central1.run.app/notes -H "Content-Type: application/json" -d "{  \"id\": \"12345\", \"title\": \"My Note\", \"text\": \"This is the content of the note 2.\", \"createdAt\": \"foo\"}"
rem curl -X PUT http://127.0.0.1:8080/notes -H "Content-Type: application/json" -d "{  \"id\": \"12345\", \"title\": \"My Note\", \"text\": \"This is the content of the note 2.\", \"createdAt\": \"2024-12-30T12:34:56.000Z\"}"

curl -X PUT http://127.0.0.1:8080/notes -H "Content-Type: application/json" -d "{\"id\":\"0\",\"title\":\"bar\",\"text\":\"foo\",\"createdAt\":\"2024-12-30T16:42:40.309Z\"}"

echo.
