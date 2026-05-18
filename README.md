## 2.1 Original code, and how it run
Program dijalankan dengan satu server dan beberapa client. Ketika salah satu client mengetik pesan, pesan tersebut dikirim ke server melalui websocket. Server kemudian membroadcast pesan itu ke client lain. Karena itu, pesan dari satu client dapat terlihat di client lainnya.
![img.png](screenshots/img.png)
![img_2.png](screenshots/img_2.png)
![img_3.png](screenshots/img_3.png)
![img_1.png](screenshots/img_1.png)
## 2.2 Modifying port
Port harus diubah pada dua sisi, yaitu server dan client. Server menggunakan port untuk membuka koneksi websocket, sedangkan client menggunakan port yang sama untuk terhubung ke server. Jika hanya salah satu yang diubah, koneksi akan gagal karena alamat tujuan tidak sesuai.