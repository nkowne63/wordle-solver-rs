# wordle-solver-rs

Rust製のwordle solverのCLIです。
起動するとCLIが立ち上がり、コマンドを使って絞り込みを行えるようになります。
基本的な戦略は平均情報量を力押しで計算しているだけです。

## コマンド一覧

- reset
- filter
- next

## コマンド詳細

### reset

CLI起動時に自動で呼ばれるコマンドです。CLIの状態をリセットし、計算に必要なデータを初期状態にリセットします。
初期状態の準備の進捗と計算にかかった時間が表示されます。

### filter word status

wordは5文字のwordです。5文字じゃなかったり小文字以外が含まれているとエラーになります。
statusは「g（緑色）」「y（黄色）」「_（灰色）」の5文字から成る文字列です。これも入力しないとエラーになります。
絞り込みの情報量と、絞り込み前後の候補の数、残り候補の最初の10語が表示されます。

### next

現在の状態から、「入力したときに得られる情報量を最大にするような単語」を計算します。
計算の進捗とかかった時間と単語、得られる平均情報量（と平均絞り込み数）が表示されます。