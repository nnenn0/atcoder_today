import sys
import webbrowser

if __name__ == '__main__':
    args = sys.argv
    abc_url = r'https://atcoder.jp/contests/abc'
    abc_number = sys.argv[1]
    abc_tasks = '/tasks'
    open_url = abc_url + abc_number + abc_tasks
    webbrowser.open(open_url)