update:
  ./update.sh

done $id:
  git add .
  git commit -m "[done] problem 000{{id}}"
  git push origin main

init $id:
  git add .
  git commit -m "[init] problem 000{{id}}"
  git push origin main
