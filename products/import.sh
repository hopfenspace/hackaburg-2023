#!/bin/bash
function upload_shop() {
  file="$1"
  name="$2"
  description="$3"
  category="$4"
  echo "Importing $name..."
  shopuuid=$(curl -H 'Content-Type: application/json' --data "{\"name\":\"$name\",\"description\":\"$description\",\"category\":\"$category\"}" http://127.0.0.1:8080/api/v1/shop | jq -r .uuid)
  if [ $? != 0 ]; then exit 1; fi
  while read json; do
    curl -H 'Content-Type: application/json' --data "${json::-1},\"shop\":\"$shopuuid\"}" http://127.0.0.1:8080/api/v1/product > /dev/null 2>&1
  done < $file
}

upload_shop "aldi.csv.json" "Aldi Süd" "ALDI SÜD und ALDI Nord gehören zu den führenden Discountern auf dem deutschen und internationalen Markt. Unsere Unternehmensgeschichte zeigt, wie wir uns mit einer einfachen Idee zu einem zuverlässigen Partner für viele Menschen weltweit entwickelt haben." "Groceries"
upload_shop "dm.csv.json" "dm" "dm ist eine deutsche Drogeriemarktkette mit Sitz in Karlsruhe. Mit rund 3.850 Filialen und 66.000 Mitarbeitern (Stand 2022) ist dm der größte Drogeriekonzern Deutschlands." "DrugStore"
upload_shop "blumen-boeswirth.csv.json" "Blumen Böswirth" "Besuchen Sie uns direkt an unserem Produktionsstandort. Wir haben immer saisonal passende Topfpflanzen für Ihr zuhause, eine große Auswahl an passenden Übertöpfen und allerlei Deko. Pflanzschalen finden Sie auch in unserer Präsentation oder fertigen wir auf Ihren Wunsch hin an." "Flowers"
upload_shop "meine-vitaminheimat.csv.json" "Meine Vitaminheimat" "Mit uns wirst du Geschmack neu entdecken. Bittere Trauben und wässrige Tomaten gehören der Vergangenheit an. Dafür sorgen wir eigens mit unseren Qualitäts- und Geschmackskontrollen." "Flowers"
