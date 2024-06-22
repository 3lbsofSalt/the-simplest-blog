# The Simplest Blog
**A blog should be simple.**
There is no need to manage a giant behemoth of a website. This blog runs off of markdown and json files.
Feel free to fork this repo to start your own blog!

## Dev Environment

Run `cargo run` and `tailwindcss -i styles/tailwind.css -o assets/main.css --watch`.

## The structure of the index files

### Projects
The index file found in `/projects/index.json` is structured as follows:
```json
{
    "projects": [
        {
            "id": "a_unique_project_id",
            "file": "file_for_project_post.md",
            "github_link": "https://github.com/3lbsofSalt/the-simplest-blog",
            "title": "The Title of Your Post",
            "thumbnail": "an/optional/thumbnail.png"
        }
    ]
}
```

- The `file` attribute points to a file specifically found in the `/projects/` directory on the top level.
- The thumbnail is optional. The file will be searched for in the `/assets/images` directory.
It can be any file which will be valid in an HTML `img` tag.
- The `id` only needs to be unique among all of the projects. It may be the same as a post.

### Posts
The index file found in `/posts/index.json` is structured as follows:
```json
{
    "available_tags": [
        { "name": "A Tag" }, 
        { "name": "Another Tag" }
    ],
    "posts": [
        {
            "id": "a_unique_post_id",
            "file": "NameOfYourFile.md",
            "title": "The Title of Your Post",
            "tags": [
                { "name": "A Tag" }
            ],
            "publish_date": "A String Representing the Publish Date",
            "thumbnail": "an/optional/thumbnail.svg"
        }
    ]
}
```

- The `file` attribute points to a file specifically found in the `/posts/` directory on the top level.
- The thumbnail is optional. The file will be searched for in the `/assets/images` directory.
It can be any file which will be valid in an HTML `img` tag.
- The `id` only needs to be unique among all of the posts. It may be the same as a project.
- While you can place tags in a post's tag entry without listing it under `available_tags`, you should list it anyway
because eventually this project will have a location with a list of all tags to search posts by. For now you can only
find the tags on the posts.
- The publish date can be any string, but it's obviously a good idea to keep them consistent between posts.
