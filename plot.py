import numpy as np
import matplotlib.pyplot as plt


data = np.loadtxt("./combined.txt")

# print(data)

# index_1 = np.where(data[:,2] == 1, data)

data2 = [el for el in data if el[2] == 1]

data1 = data[(data[:,2] == 1)]
data2 = data[(data[:,2] == 2)]
data3 = data[(data[:,2] == 3)]


plt.scatter(data1[:,0], data1[:,1], color='red', s=1.0)
plt.scatter(data2[:,0], data2[:,1], color='black', s=1.0)
plt.scatter(data3[:,0], data3[:,1], color='blue', s=1.0)


plt.show()

# print(index_1)

# data1 = data.index()



# plt.plot(numbers)

# plt.show()


# print("hello world")