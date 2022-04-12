# Rust simple-template
Getting comfortable with Rust by making a (terrible) CLI templating tool


### Installation via cargo
```
git clone https://github.com/kremrik/simple-template
cd rust
cargo install --path .
```


### Dynamic template help example
```
$ cat ../templates/base2.html | simple-template --help --bgn '<<' --end '>>'
NAME
	simple-template

SYNOPSIS
	cat <template> | simple-template [OPTION]...

DESCRIPTION
	--head
		/path/to/head
	--body
		/path/to/body
```


### Templating example
```
$ cat ../templates/base.html | simple-template --body ../templates/body.html --head ../templates/head.html 
<!DOCTYPE html>
<html lang="en">
   <head>
     <link rel="stylesheet" href="styles.css">
   </head>
   <div>
     <p>
       This is a test paragraph.
     </p>
   </div>
</html>
```


### Composing templating and help
```
$ cat ../templates/base.html | simple-template --body ../templates/body.html | simple-template -h
NAME
	simple-template

SYNOPSIS
	cat <template> | simple-template [OPTION]...

DESCRIPTION
	--head
		/path/to/head
```
