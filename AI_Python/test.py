import subprocess

# 'your_cui_software' を実際のCUIソフトのコマンドに置き換える
process = subprocess.Popen(['your_cui_software'], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, shell=True)

# CUIにコマンドを送信
command = 'your_command_to_send\n'  # ここに実行したいコマンドを指定
process.stdin.write(command.encode('utf-8'))
process.stdin.flush()

# 出力結果を取得
output, error = process.communicate()

# 結果を表示
print(output.decode('utf-8'))

# エラーがあれば表示
if error:
    print(error.decode('utf-8'))
