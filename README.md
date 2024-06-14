<a name="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h3 align="center">type-erased-table</h3>
  <div align="center">
    <a href="https://crates.io/crates/type-erased-table"><img src="https://img.shields.io/crates/v/type-erased-table.svg?label=type-erased-table" alt="crates.io"></a>
    <a href="https://docs.rs/type-erased-table"><img src="https://docs.rs/type-erased-table/badge.svg" alt="docs.rs"></a>
  </div>
  <p align="center">
    A column-oriented based raw data storage
    <br />
    <a href="https://github.com/ValentinRio/type-erased-table/issues">Report Bug</a>
  </p>
</div>



<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

Provide a Table struct that allows to store rows of data composed of different types.
This structure can be represented as a [Structure of arrays](https://en.wikipedia.org/wiki/AoS_and_SoA) and is built has an HashMap where keys are column identifiers and values are Column. Each Column is a contiguous array and every row component can be accessed with the same index on each Column.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



### Built With

* Nohash to create hashmap without hashing algorithm (keys are u32) - https://github.com/tetcoin/nohash
* Based on bevy BlobVec implementation - https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- GETTING STARTED -->
## Getting Started

This is an example of how you use this crate to to create a table struct with a simple columns.

### Installation

Add dependency to you cargo.toml

```toml
[dependencies]
type-erased-table = "0.1.0"
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- USAGE EXAMPLES -->
## Usage

After creating a table just add a column and start filling it with un-typed data.

```rust
let mut table = Table::new(1);

let value: u32 = 2;

// Get Layout from concrete type
let layout = Layout::for_value(&value);

// Create column with u32 identifier
let column_info = ColumnInfo::new(1, layout);

table.add_column(column_info);

let column = table.get_column_mut(1).unwrap();

// Push data into row of column by passing pointer as *const u8
unsafe { column.push(ptr::addr_of!(value) as *const u8) }

// Read value of row 0
let row_ptr = unsafe { column.get(0) };
let row_value: u32 = unsafe { (*row_ptr).into() };
```

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request.
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- LICENSE -->
## License

Distributed under the MIT License.

<p align="right">(<a href="#readme-top">back to top</a>)</p>



<!-- CONTACT -->
## Contact

Project Link: [https://github.com/ValentinRio/type-erased-table](https://github.com/ValentinRio/type-erased-table)

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->
[issues-url]: https://github.com/ValentinRio/type-erased-table/issues
[license-url]: https://github.com/ValentinRio/type-erased-table/blob/main/LICENSE.txt
