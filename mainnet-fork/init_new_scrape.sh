rm accounts/* 
echo "SCRAPING ACCOUNTS RUNNING" &&
npx ts-node new_scrape.ts &&
echo "RUNNING SETUP VALIDATOR.PY" &&
python3 setup_validator.py &&
echo "RUNNING START LOCALNET.SH" &&
bash start_localnet.sh