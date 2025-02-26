# DataWeave

**DataWeave** is a CLI tool that converts **JSON/YAML to CSV** by generating all possible **permutations** of the provided data.\
It’s built in **Rust** and optimized for high-performance structured data transformation.

## 🚀 Features

TBD

---

## 📛 Installation

Install **DataWeave** via **Cargo**:

```sh
cargo install dataweave
```

Or, if you are working with the repository:

```sh
git clone https://github.com/MadeByAe/dataweave.git
cd dataweave
cargo build --release
```

---

## 🛠️ Usage

### **Basic Usage**

Convert a JSON or YAML file into a CSV:

```sh
dataweave -f input.yaml
```

This will generate a `` file in the current directory.

### **Custom Output File**

Specify where to save the CSV:

```sh
dataweave -f input.json -o output.csv
```

---

## 📜 Input Format

### ✅ **JSON Example

```json
{
  "make": ["BMW", "Audi", "Porsche"],
  "color": ["Silver", "Black", "White"],
  "interior": ["Light", "Dark"]
}

```

### ✅ **YAML Example

```yaml
make:
  - Audi
  - BMW
  - Porsche

exterior_color:
  - Silver
  - Black
  - White

interior_color:
  - Light
  - Dark
```

---

## 📊 Output (CSV)

For the above JSON/YAML, the output **CSV file** will contain all **possible permutations**:

```csv
make,exterior_color,interior_color
Audi,Silver,Light
Audi,Silver,Dark
Audi,Black,Light
Audi,Black,Dark
Audi,White,Light
Audi,White,Dark
BMW,Silver,Light
BMW,Silver,Dark
BMW,Black,Light
BMW,Black,Dark
BMW,White,Light
BMW,White,Dark
Porsche,Silver,Light
Porsche,Silver,Dark
Porsche,Black,Light
Porsche,Black,Dark
Porsche,White,Light
Porsche,White,Dark
```

---

## 🛠️ Options

| Flag           | Description                                       | Example         |
| -------------- | ------------------------------------------------- | --------------- |
| `-f, --file`   | Input file (JSON/YAML)                            | `-f data.yaml`  |
| `-o, --output` | Output CSV file (default: `output.csv`) | `-o output.csv` |

---

## Contributing
Contributions to DataWeave are welcome! If you have suggestions for improvements or additional utility modules, please feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/MadeByAe/DataWeave).

## License
This project is licensed under the [MIT License](LICENSE.md). Feel free to use, modify, and distribute this code according to the terms of the license.

## About
DataWeave is maintained by (Ae) Angel Estrada. For questions or support, please contact [angel-estrada.com](https://www.angel-estrada.com).

---

Made with ❤️ in San Francisco
