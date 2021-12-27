import oanda_coms_lib as ocl
import datetime
import time

# intial trade cleanup

open_trades = ocl.get_open_trades()
ids = sorted(list(open_trades.keys()), key=lambda item : str(item))


for id in ids:
    print(id, open_trades[id])
    ocl.remove_order(int(open_trades[id]['currentUnits']), int(id))

delta_hour = 0
time_mod = 30

open_trade_id = None


print("active")
while True:
    curr_minute = datetime.datetime.now().minute

    if (curr_minute % time_mod) ==  0:
        if open_trade_id != None:
            print(ocl.remove_order(int(open_trade_id), int(open_trade_id)), datetime.datetime.now())
        open_trade_id = ocl.post_order("EUR_USD", 100, "MARKET")

    time.sleep(60)
