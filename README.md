# stellar-triet
<img width="1121" height="672" alt="image" src="https://github.com/user-attachments/assets/7f0ccc7b-9e74-4952-9a6d-101d929ad6e7" />
📦 PiggyBank Smart Contract (Stellar Soroban)
🧠 Mô tả

PiggyBank là một smart contract đơn giản trên Stellar cho phép lưu trữ số dư như một “heo đất”. Người dùng có thể nạp tiền vào contract, nhưng chỉ chủ sở hữu (owner) mới có quyền rút tiền.

⚙️ Chức năng chính
init(owner)
Khởi tạo contract với địa chỉ owner.
deposit(amount)
Nạp tiền vào contract (tăng số dư).
withdraw(to, amount)
Rút tiền khỏi contract (chỉ owner được phép).
get_balance()
Xem số dư hiện tại trong contract.
🔐 Quyền hạn
Mọi người có thể deposit
Chỉ owner mới có thể withdraw
⚠️ Lưu ý
Contract hiện chỉ mô phỏng số dư nội bộ, chưa tích hợp chuyển token thật.
Dùng cho mục đích học tập và demo.
🚀 Contract ID
CCVJDFE6TTCR7SGJIWR6ESV3BYCE4I3H2YRUZIF4EKJVMQAPYKDWZ3M4
