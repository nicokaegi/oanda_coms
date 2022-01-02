import matplotlib.pyplot as plt

import oanda_coms_lib as ocl
import pandas as pd
import numpy as np

import plotly.graph_objects as go

import datetime
import time


open_trades = ocl.get_open_trades()
ids = sorted(list(open_trades.keys()), key=lambda item : str(item))


for id in ids:
    print(id, open_trades[id])
    ocl.remove_order(int(open_trades[id]['currentUnits']), int(id))


data = ocl.get_instrument_candles("EUR_USD",1200,"D")
#data = ocl.get_instrument_candle_range("EUR_USD","2020-10-12T07:20:50.52Z","2020-10-20T07:20:50.52Z","M5")

print(" created order ")

open_trade_id = ocl.post_order("EUR_USD", 100, "MARKET")


print(" remove : ", open_trade_id)

ocl.remove_order(int(open_trade_id['units']), int(open_trade_id['id']))

time.sleep(5)

print(" print open trades ")

open_trades = ocl.get_open_trades()
print(open_trades)
'''
thing = pd.DataFrame(data)
print(thing)

fig = go.Figure(data=[go.Candlestick(x=thing['time'],
                open=thing['open'],
                high=thing['high'],
                low=thing['low'],
                close=thing['close'])])

fig.show()
'''
#thing['time'] = thing['time'].map(lambda item : pd.Timestamp(item))

#thing['diff'] = thing['open'].apply(lambda item : float(item)) - thing['close'].apply(lambda item : float(item))


'''
neg = []
pos = []

for item in thing['diff']:
    if  0 < item:
        pos.append(item)
    else:
        neg.append(item)

pos = np.array(pos)
neg = np.array(neg)


print(" pos : ", pos.mean(), " neg : ", neg.mean())

plt.plot(thing['time'], thing['diff'])
plt.axhline(y = pos.mean(), color = 'r', linestyle = '-')
plt.axhline(y = neg.mean(), color = 'r', linestyle = '-')
plt.show()
'''
