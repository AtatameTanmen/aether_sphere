# AetherSphere

AetherSphere は、球体表面上で流体（風や水）の挙動をシミュレートし、リアルタイムに可視化するプロジェクトです。  
物理の厳密な再現ではなく、眺めて楽しめるような環境構築を目指しています。  
将来的には、惑星規模での気候や生命活動のシミュレートに応用できればと考えています。

---

## What this project does

* 球体表面上での流体シミュレーション（浅水方程式）
* GPUコンピュートシェーダによる計算
* シミュレーション結果のリアルタイム可視化

---

## Tech Stack

* Rust
* Bevy
* Compute Shader (GPU)
* Icosphere / Voronoi mesh

---

## Status

初期段階のプロジェクトです。  
現在は以下に取り組んでいます：

* 球体メッシュの生成と描画
* カメラ操作とデバッグ表示

---

## Roadmap

### Phase 1

* 球体メッシュの生成と描画
* カメラ操作とデバッグ表示

### Phase 2

* GPUコンピュートシェーダによる計算
* 高さ場の可視化

### Phase 3

* 粘性などによる安定性の向上
* 可視化の改善

---

## References

* [A unified approach to energy conservation and potential vorticity dynamics for arbitrarily structured C-grids](https://doi.org/10.1016/j.jcp.2009.12.007)  
  任意構造の C-grid におけるエネルギー保存とポテンシャル渦度に関する研究
