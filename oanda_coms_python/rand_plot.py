import matplotlib.pyplot as plt

import oanda_coms_lib as ocl
import pandas as pd

data = ocl.get_instrument_candles("EUR_USD",25000,"M5")
#data = ocl.get_instrument_candle_range("EUR_USD","2020-10-12T07:20:50.52Z","2020-10-20T07:20:50.52Z","M5")

thing = pd.DataFrame(data)
thing['time'] = thing['time'].map(lambda item : pd.Timestamp(item))

thing['diff'] = thing['open'].apply(lambda item : float(item)) - thing['close'].apply(lambda item : float(item))

plt.plot(thing['time'], thing['diff'])
plt.show()
