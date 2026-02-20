#!/bin/sh

curl "http://localhost:3000/api/todos?status=open&q=ea" \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjEsImV4cCI6MTc3MTY5ODI2NH0.yNJgXUEPnGwNtiu3FRte9z21C77SvQCDWC4egxeh78s"
