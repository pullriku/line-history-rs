import datetime

with open("history.txt", "w") as f:
    f.write("[LINE] xxxのトーク履歴\n保存日時：2024/01/01 00:00\n\n")
    
    for i in range(10_000):
        date = datetime.datetime(2024, 1, 1, 0, 0) + datetime.timedelta(days=i)
        f.write(f"{date:%Y/%m/%d(%a)}\n")
        for j in range(100):
            time = date + datetime.timedelta(minutes=j)
            f.write(f"{time:%H:%M}\tNAME\tMESSAGE\n")
        f.write(f"{time:%H:%M}\tNAME\t\"MESSAGE LINE 0\nMESSAGE LINE 1\nMESSAGE LINE 2\nMESSAGE LINE 3\nMESSAGE LINE 4\nMESSAGE LINE 5\nMESSAGE LINE 6\nMESSAGE LINE 7\nMESSAGE LINE 8\"\n")
        f.write("\n")
