import oanda_coms_lib as ocl
import datetime
import time
import traceback

''' 

yet another test script but this time for a basic trading bot in python

all is does is but and sell orders in regular intervels. 

''' 

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
            try:
                print("sell", ocl.remove_order(int(last_transaction['units']), int(last_transaction['id'])), datetime.datetime.now())

            except:
                traceback.print_exc()

        last_transaction = ocl.post_order("EUR_USD", 100, "MARKET")
        print("buy",last_transaction)

    time.sleep(60)
