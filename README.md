## Prelude

Hi! This is my Bachelor's Project, but you are free to do wharever you want (#MIT License).

(All development was tested only in Linux env, which mean idk if this repo would work under Windows/Mac or other Unix systems)

* for backend I use Rust framework - Axum (242 dependencies - Mar 27 2024).
* for frontend I use Rust framework - Yew (326 dependencies - Mar 27 2024).

![Home Page](/images/preview.png)
<p align="center">
    Image 1 - Home page (buttons have no login for now)
</p>

## REPOSITORY GUIDE
* ##### /frontent - the root of frontent.
* ##### /images - folder for repo images, NOT used in the website itself.
* ##### /migration - migrations for the database.
* ##### /public - folder for images, used in the webiste itself.
* ##### /scripts - folder for production automotion, configs, etc.
* ##### /server - the root of backend.
* ##### /shared - models for data representation, used in front and back.



## TO-DO 

#### Frontend

- [x] visual header
    - [ ] workable header
    - [ ] locales

- [x] visual footer
    - [ ] workable footer
    - [x] locales

- [ ] pages


#### Backend

- [x] tls support (.pem)
- [x] run with commands

#### Database

- [x] workable

#### Other

- [x] migration support
    - [ ] migration scripting

- [x] docker-compose for fullstack
    - [ ] docker-compose scripting

- [x] nginx config

- [x] shared models as a lib in workspace