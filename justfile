update:
  ./update.sh

done $id:
  git add .
  git commit -m "[done] {{id}}"
  git push origin main
